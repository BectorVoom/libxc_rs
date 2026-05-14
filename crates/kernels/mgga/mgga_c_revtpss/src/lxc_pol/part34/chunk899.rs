//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 899/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk899<F: Float>(t23363: F, t23382: F, t868: F, t225: F, t23359: F, t10501: F, t10503: F, t10984: F, t14474: F, t14486: F, t14998: F, t15004: F, t15006: F, t15015: F, t18318: F, t213: F, t257: F, t4474: F, t6049: F, t6072: F, t865: F) -> (F, F, F) {
    let t23383 = t23363 + t23382;
    let t23384 = t868 * t23383;
    let t23388 = t23359 * t225;
    let t23400 = -0.19514881078765566038e-2 * t14474 + 0.39029762157531132076e-1 * t14486 - 0.65854491829355115987e0 * t865 * t23384 - 0.16463622957338778996e-1 * t18318 + 0.65854491829355115987e0 * t213 * t23388 * t257 + t10501 - 0.21951497276451705329e-1 * t14998 - t10503 - 0.19756347548806534796e1 * t4474 * t6072 + 0.39512695097613069591e1 * t4474 * t6049 - 0.34697458558045176417e-2 * t15004 + t10984 - 0.39029762157531132076e-1 * t15006 + 0.19514881078765566038e-2 * t15015;
    (t23383, t23384, t23400)
}
