//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1023/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1023<F: Float>(t27235: F, t6317: F, t4203: F, t1471: F, t2059: F, t21156: F, t14491: F, t4271: F, t7706: F, t4265: F, t8220: F, t14409: F, t1460: F, t1470: F, t21152: F, t21252: F, t2225: F, t2242: F, t26568: F, t26573: F, t26605: F, t26697: F, t4253: F, t476: F, t5949: F, t5958: F, t6247: F, t6256: F, t7865: F, t7873: F, t7878: F, t7898: F) -> (F, F) {
    let t27236 = t6317 * t27235;
    let t27237 = t4203 * t27236;
    let t27261 = t1471 * t21156 * t2059;
    let t27265 = t4271 * t14491 * t7706;
    let t27270 = t4265 * t8220;
    let t27272 = -t14409 + 0.11791604938271604938e-1 * t21152 + 0.46434375e-2 * t6256 * t26568 - 0.9286875e-2 * t4253 * t26573 + 0.1857375e-1 * t4253 * t26605 + 0.123825e-1 * t1460 * t7878 - 0.619125e-2 * t1460 * t7898 - 0.123825e-1 * t6247 * t2225 - 0.123825e-1 * t2242 * t5958 + 0.46434375e-2 * t1460 * t7865 + 0.9286875e-2 * t1460 * t7873 + 0.9286875e-2 * t476 * t26697 - 0.53062222222222222222e-1 * t1470 * t27261 - 0.44218518518518518518e-1 * t1470 * t27265 - 0.1857375e-1 * t21252 * t5949 + 0.35374814814814814815e-1 * t27270;
    (t27237, t27272)
}
