//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1168/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1168<F: Float>(t27619: F, t8513: F, t1043: F, t1089: F, t1096: F, t120321: F, t120322: F, t120376: F, t120452: F, t120555: F, t120569: F, t120724: F, t27651: F, t27688: F, t3046: F, t31891: F, t31892: F, t31894: F, t31903: F, t32014: F, t32015: F, t32016: F, t33765: F, t33774: F, t33803: F, t33811: F, t4583: F, t4940: F, t4975: F, t7135: F, t8507: F, t99638: F, t999: F) -> (F, F) {
    let t126852 = t8513 * t27619;
    let t126868 = F::new(0.22847895066040941046e1) * t120724 * t33811 * t1043 * t1089 + F::new(0.22847895066040941046e1) * t120724 * t27651 * t4975 * t7135 + F::new(0.18822977838986977999e-3) * t32014 * t32015 * t32016 * t4583 - F::new(0.56468933516960933998e-3) * t120376 * t32015 * t120322 * t99638 - F::new(0.10038921514126388266e-2) * t120555 * t33765 + F::new(0.56468933516960933998e-3) * t120321 * t32015 * t120322 * t4940 + F::new(0.11423947533020470523e1) * t126852 * t31894 + F::new(0.6854368519812282314e1) * t31891 * t120452 * t33803 * t1096 - F::new(0.34271842599061411569e1) * t31903 * t31892 * t33811 * t999 - F::new(0.17347256376410398924e1) * t3046 * t8507 * t33774 + F::new(0.34694512752820797848e1) * t120569 * t27688;
    (t126852, t126868)
}
