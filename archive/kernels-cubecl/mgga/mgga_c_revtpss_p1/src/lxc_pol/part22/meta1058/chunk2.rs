//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3755/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3755<F: Float>(t3717: F, t70994: F, t1122: F, t1250: F, t12866: F, t1715: F, t17353: F, t17539: F, t17651: F, t17661: F, t17673: F, t17736: F, t17737: F, t17800: F, t20721: F, t20959: F, t3568: F, t3626: F, t3723: F, t44521: F, t44925: F, t44931: F, t57631: F, t57663: F, t59142: F, t59144: F, t59146: F, t70496: F) -> F {
    let t71513 = t3717 * t70994;
    let t71527 = t44925 / F::cast_from(1296.0_f64) + F::cast_from(0.28582678745379824648e-3_f64) * t12866 * t17661 * t17673 - F::cast_from(5.0_f64) / F::cast_from(1944.0_f64) * t44931 + F::cast_from(0.25724410870841842184e-2_f64) * t57631 * t20959 - F::cast_from(0.11433071498151929859e-2_f64) * t17736 * t3626 * t20721 * t1122 - F::cast_from(0.57165357490759649296e-3_f64) * t17736 * t3626 * t17737 * t17539 - F::cast_from(0.57165357490759649296e-3_f64) * t59142 - F::cast_from(0.14481890564325777821e-1_f64) * t71513 * t3723 - F::cast_from(5.0_f64) / F::cast_from(243.0_f64) * t59144 + F::cast_from(0.57165357490759649296e-3_f64) * t57663 * t17651 - F::cast_from(0.11433071498151929859e-2_f64) * t70496 * t17800 - F::cast_from(0.57165357490759649296e-3_f64) * t44521 * t17353 * t1250 * t1715 * t3568 - F::cast_from(0.57165357490759649296e-3_f64) * t59146;
    t71527
}
