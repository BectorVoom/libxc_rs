//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1114/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1114<F: Float>(t54719: F, t54724: F, t1167: F, t2494: F, t1105: F, t3324: F, t15382: F, t2053: F, t3931: F, t944: F, t3928: F, t3717: F, t1172: F, t810: F, t12263: F, t13756: F, t14153: F, t14390: F, t14831: F, t15118: F, t3189: F, t3946: F, t4062: F, t4063: F, t50818: F, t52789: F, t52816: F) -> (F, F, F) {
    let t55984 = 35.0 / 108.0 * t54719;
    let t55986 = 119.0 / 6912.0 * t54724;
    let t56018 = t2494 * t1167;
    let t56027 = t1105 * t3324;
    let t56031 = t15382 * t2053;
    let t56034 = t1167 * t3324;
    let t56038 = t3931 * t944;
    let t56042 = t3928 * t944;
    let t56046 = t3717 * t944;
    let t56053 = t1172 * t810;
    let t56056 = -t12263 * t4062 * t4063 + 12.0 * t13756 * t14390 * t3189 + 4.0 * t14153 * t4062 * t56034 + 2.0 * t14153 * t4062 * t56042 + 4.0 * t14831 * t4062 * t52816 - 6.0 * t3946 * t4063 * t56018 - 6.0 * t3946 * t4063 * t56027 - 3.0 * t3946 * t4063 * t56046 - 6.0 * t4062 * t50818 * t56038 - t4062 * t56031 * t944 + 6.0 * t15118 * t56053 - t52789;
    (t55984, t55986, t56056)
}
