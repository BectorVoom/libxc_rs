//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1214/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1214(t10667: f64, t835: f64, t723: f64, t10718: f64, t7137: f64, t10779: f64, t10784: f64, t10789: f64, t1716: f64, t1897: f64, t1901: f64, t2508: f64, t2580: f64, t32159: f64, t32161: f64, t32167: f64, t32169: f64, t32172: f64, t32173: f64, t32180: f64, t32185: f64, t32186: f64, t7129: f64) -> (f64, f64, f64) {
    let t32190 = t835 * t10667;
    let t32191 = t32190 * t723;
    let t32201 = 0.41016139894091862846e-1_f64 * t7137 * t10718;
    let t32202 = t32159 + t32161 - t32167 + t32169 + t32172 + 0.76905262301422242837e-2_f64 * t1897 * t1901 * t32173 + 0.15381052460284448567e-1_f64 * t7129 * t10779 + 0.15381052460284448567e-1_f64 * t1897 * t1901 * t32180 - t32185 - 0.46143157380853345702e-1_f64 * t2508 * t1901 * t32186 + 0.30762104920568897134e-1_f64 * t2508 * t2580 * t32191 - 0.23071578690426672851e-1_f64 * t2508 * t10789 * t1716 + 0.41016139894091862847e-1_f64 * t7137 * t10784 + t32201;
    (t32190, t32191, t32202)
}
