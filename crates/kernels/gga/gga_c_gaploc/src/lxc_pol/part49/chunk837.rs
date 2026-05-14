//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 837/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk837<F: Float>(t39695: F, t6520: F, t6525: F, t7888: F, t2326: F, t3394: F, t6514: F, t9074: F, t30204: F, t31769: F, t10177: F, t19531: F, t883: F, t1358: F, t23915: F, t42195: F) -> (F, F, F, F, F, F) {
    let t42638 = 0.63233348079280332443e-2 * t39695;
    let t42640 = t6525 * t7888 * t6520;
    let t42641 = 0.71137516589190373998e-2 * t42640;
    let t42644 = t9074 * t6514 * t3394 * t2326;
    let t42645 = 0.16598753870811087267e-1 * t42644;
    let t42647 = t9074 * t30204 * t31769;
    let t42648 = 0.284550066356761496e-1 * t42647;
    let t42651 = t9074 * t19531 * t883 * t10177;
    let t42652 = 0.142275033178380748e-1 * t42651;
    let t42655 = 0.18970004423784099732e-1 * t1358 * t23915 * t42195;
    (t42638, t42641, t42645, t42648, t42652, t42655)
}
