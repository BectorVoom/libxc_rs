//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1169/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1169<F: Float>(t3268: F, t42859: F, t11627: F, t126659: F, t3153: F, t33787: F, t73: F, t100743: F, t120184: F, t120218: F, t120223: F, t120376: F, t120397: F, t120507: F, t120569: F, t120578: F, t126442: F, t1646: F, t27557: F, t27664: F, t3092: F, t3116: F, t3143: F, t31903: F, t31959: F, t31981: F, t31993: F, t33749: F, t33803: F, t359: F, t4783: F, t4983: F, t4998: F, t5015: F, t8508: F, t8514: F, t906: F, t988: F, t999: F) -> (F, F) {
    let t126891 = t42859 * t3268;
    let t126892 = t126891 * t11627;
    let t126894 = t126659 * t3153;
    let t126903 = t33787 * t73;
    let t126915 = -F::cast_from(0.11156198762715303246e-2_f64) * t120184 * t31993 * t3116 * t1646 * t988 + F::cast_from(0.16734298144072954869e-2_f64) * t120218 * t31993 * t3116 * t126442 + F::cast_from(0.11156198762715303246e-2_f64) * t120223 * t31993 * t3116 * t100743 + F::cast_from(0.37645955677973955999e-3_f64) * t120376 * t3092 * t33749 * t906 + F::cast_from(0.18822977838986977999e-3_f64) * t120578 + F::cast_from(0.17347256376410398924e1_f64) * t8508 * t31981 * t359 * t5015 + F::cast_from(0.34271842599061411569e1_f64) * t8514 * t126892 * t126894 * t4983 - F::cast_from(0.11423947533020470523e1_f64) * t8514 * t126891 * t3143 * t126894 * t4998 + F::cast_from(0.17347256376410398924e1_f64) * t120507 * t126903 * t27664 + F::cast_from(0.51407763898592117355e1_f64) * t31903 * t31959 * t33803 * t999 - F::cast_from(0.24791552806034007213e-3_f64) * t120397 * t4783 + F::cast_from(0.34694512752820797848e1_f64) * t120569 * t27557;
    (t126903, t126915)
}
