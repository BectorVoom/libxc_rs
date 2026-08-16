//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1169/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1169(t3268: f64, t42859: f64, t11627: f64, t126659: f64, t3153: f64, t33787: f64, t73: f64, t100743: f64, t120184: f64, t120218: f64, t120223: f64, t120376: f64, t120397: f64, t120507: f64, t120569: f64, t120578: f64, t126442: f64, t1646: f64, t27557: f64, t27664: f64, t3092: f64, t3116: f64, t3143: f64, t31903: f64, t31959: f64, t31981: f64, t31993: f64, t33749: f64, t33803: f64, t359: f64, t4783: f64, t4983: f64, t4998: f64, t5015: f64, t8508: f64, t8514: f64, t906: f64, t988: f64, t999: f64) -> (f64, f64) {
    let t126891 = t42859 * t3268;
    let t126892 = t126891 * t11627;
    let t126894 = t126659 * t3153;
    let t126903 = t33787 * t73;
    let t126915 = -0.11156198762715303246e-2_f64 * t120184 * t31993 * t3116 * t1646 * t988 + 0.16734298144072954869e-2_f64 * t120218 * t31993 * t3116 * t126442 + 0.11156198762715303246e-2_f64 * t120223 * t31993 * t3116 * t100743 + 0.37645955677973955999e-3_f64 * t120376 * t3092 * t33749 * t906 + 0.18822977838986977999e-3_f64 * t120578 + 0.17347256376410398924e1_f64 * t8508 * t31981 * t359 * t5015 + 0.34271842599061411569e1_f64 * t8514 * t126892 * t126894 * t4983 - 0.11423947533020470523e1_f64 * t8514 * t126891 * t3143 * t126894 * t4998 + 0.17347256376410398924e1_f64 * t120507 * t126903 * t27664 + 0.51407763898592117355e1_f64 * t31903 * t31959 * t33803 * t999 - 0.24791552806034007213e-3_f64 * t120397 * t4783 + 0.34694512752820797848e1_f64 * t120569 * t27557;
    (t126903, t126915)
}
