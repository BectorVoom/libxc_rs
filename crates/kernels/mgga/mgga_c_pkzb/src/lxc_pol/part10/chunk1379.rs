//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1379/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1379<F: Float>(t10000: F, t10003: F, t22567: F, t22750: F, t2297: F, t2312: F, t2318: F, t27400: F, t27402: F, t27406: F, t27408: F, t27411: F, t27414: F, t27417: F, t27420: F, t27423: F, t3807: F, t3823: F, t6272: F, t6282: F, t6308: F, t8071: F, t8107: F, t8120: F, t8129: F, t8139: F, t8164: F, t8167: F, t8174: F, t8178: F, t8211: F) -> (F,) {
    let t27573 = -8.0 * t8211 * t8129 + 0.12865583598954028054e3 * t8120 * t8139 + 12.0 * t6308 * t10000 - 8.0 * t6272 * t10003 - 0.23392894490538584828e1 * t8071 * t8164 - 0.2077903092681775651e3 * t22567 * t8167 + 0.34631718211362927517e2 * t8107 * t8174 + 0.20508037716432813315e4 * t22750 * t8178 + 0.35089341735807877242e1 * t2318 * t3807 * t2312 + 0.6233709278045326953e3 * t6282 * t3823 * t2297 + t27400 + t27402 - t27406 - t27408 - t27411 - t27414 + t27417 + t27420 + t27423;
    (t27573,)
}
