//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1174/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1174<F: Float>(t3336: F, t33836: F, t1100: F, t1102: F, t120745: F, t120749: F, t120767: F, t126471: F, t126508: F, t126546: F, t126596: F, t126640: F, t126673: F, t126708: F, t126741: F, t126786: F, t126828: F, t126868: F, t126915: F, t126948: F, t126995: F, t127035: F, t127074: F, t1699: F, t198: F, t25709: F, t25713: F, t27708: F, t27717: F, t32030: F, t32036: F, t336: F, t5019: F, t5023: F, t7177: F, t7181: F, t7840: F) -> F {
    let t127082 = t33836 * t3336;
    let t127112 = t198 * t336 * (t126471 + t126508 + t126546 + t126596 + t126640 + t126673 + t126708 + t126741 + t126786 + t126828 + t126868 + t126915 + t126948 + t126995 + t127035 + t127074) * t1102 - t5023 * t127082 * t1100 - t5023 * t120745 * t1699 + F::cast_from(2.0_f64) * t5023 * t120749 * t27717 - t5023 * t32030 * t5019 - F::cast_from(2.0_f64) * t5023 * t25709 * t7840 + F::cast_from(4.0_f64) * t5023 * t25713 * t7840 * t1100 - F::cast_from(2.0_f64) * t5023 * t7181 * t27708 + F::cast_from(4.0_f64) * t5023 * t25713 * t1699 * t7177 - F::cast_from(6.0_f64) * t5023 * t120767 * t27717 + F::cast_from(2.0_f64) * t5023 * t32036 * t5019;
    t127112
}
