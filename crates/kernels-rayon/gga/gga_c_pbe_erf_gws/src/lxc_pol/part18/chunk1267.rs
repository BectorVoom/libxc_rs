//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1267/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1267(t15382: f64, t2053: f64, t1167: f64, t3324: f64, t3931: f64, t944: f64, t3928: f64, t3717: f64, t1172: f64, t810: f64, t12263: f64, t13756: f64, t14153: f64, t14390: f64, t14831: f64, t15118: f64, t3189: f64, t3946: f64, t4062: f64, t4063: f64, t50818: f64, t52789: f64, t52816: f64, t56018: f64, t56027: f64) -> f64 {
    let t56031 = t15382 * t2053;
    let t56034 = t1167 * t3324;
    let t56038 = t3931 * t944;
    let t56042 = t3928 * t944;
    let t56046 = t3717 * t944;
    let t56053 = t1172 * t810;
    let t56056 = -t12263 * t4062 * t4063 + 12.0_f64 * t13756 * t14390 * t3189 + 4.0_f64 * t14153 * t4062 * t56034 + 2.0_f64 * t14153 * t4062 * t56042 + 4.0_f64 * t14831 * t4062 * t52816 - 6.0_f64 * t3946 * t4063 * t56018 - 6.0_f64 * t3946 * t4063 * t56027 - 3.0_f64 * t3946 * t4063 * t56046 - 6.0_f64 * t4062 * t50818 * t56038 - t4062 * t56031 * t944 + 6.0_f64 * t15118 * t56053 - t52789;
    t56056
}
