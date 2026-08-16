//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1168/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1168(t27619: f64, t8513: f64, t1043: f64, t1089: f64, t1096: f64, t120321: f64, t120322: f64, t120376: f64, t120452: f64, t120555: f64, t120569: f64, t120724: f64, t27651: f64, t27688: f64, t3046: f64, t31891: f64, t31892: f64, t31894: f64, t31903: f64, t32014: f64, t32015: f64, t32016: f64, t33765: f64, t33774: f64, t33803: f64, t33811: f64, t4583: f64, t4940: f64, t4975: f64, t7135: f64, t8507: f64, t99638: f64, t999: f64) -> (f64, f64) {
    let t126852 = t8513 * t27619;
    let t126868 = 0.22847895066040941046e1_f64 * t120724 * t33811 * t1043 * t1089 + 0.22847895066040941046e1_f64 * t120724 * t27651 * t4975 * t7135 + 0.18822977838986977999e-3_f64 * t32014 * t32015 * t32016 * t4583 - 0.56468933516960933998e-3_f64 * t120376 * t32015 * t120322 * t99638 - 0.10038921514126388266e-2_f64 * t120555 * t33765 + 0.56468933516960933998e-3_f64 * t120321 * t32015 * t120322 * t4940 + 0.11423947533020470523e1_f64 * t126852 * t31894 + 0.6854368519812282314e1_f64 * t31891 * t120452 * t33803 * t1096 - 0.34271842599061411569e1_f64 * t31903 * t31892 * t33811 * t999 - 0.17347256376410398924e1_f64 * t3046 * t8507 * t33774 + 0.34694512752820797848e1_f64 * t120569 * t27688;
    (t126852, t126868)
}
