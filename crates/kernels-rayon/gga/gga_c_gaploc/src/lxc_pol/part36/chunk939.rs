//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 939/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk939(t1843: f64, t32261: f64, t7064: f64, t2558: f64, t33360: f64, t9647: f64, t13194: f64, t1841: f64, t13168: f64, t270: f64, t43028: f64, t43032: f64, t43035: f64, t43040: f64, t43043: f64, t43046: f64, t43049: f64, t43051: f64, t43053: f64, t43054: f64, t43055: f64, t43082: f64, t43087: f64, t650: f64, t681: f64, t738: f64) -> f64 {
    let t43090 = t7064 * t1843 * t32261;
    let t43093 = t9647 * t33360 * t2558;
    let t43094 = 0.64087718584518535698e-3_f64 * t43093;
    let t43095 = t1841 * t13194;
    let t43096 = 0.17090058289204942852e-2_f64 * t43095;
    let t43097 = t43028 + t43032 - 0.17090058289204942852e-2_f64 * t43035 - t43040 + t43043 + 0.51270174867614828558e-2_f64 * t43046 - t43049 - 0.46143157380853345702e-1_f64 * t43051 - t43053 + t43054 - t43055 - 0.10254034973522965712e-1_f64 * t650 * t13168 - 0.76905262301422242837e-2_f64 * t681 * t13168 - 0.76905262301422242837e-2_f64 * t270 * t738 * t43082 + 0.15381052460284448567e-1_f64 * t43087 + 0.64087718584518535698e-3_f64 * t43090 + t43094 - t43096;
    t43097
}
