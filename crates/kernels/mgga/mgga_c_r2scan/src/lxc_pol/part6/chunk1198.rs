//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1198/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1198<F: Float>(t14: F, t175: F, t21066: F, t5670: F, t5675: F, t1907: F, t5448: F, t645: F, t1399: F, t1932: F, t1956: F, t2008: F, t2029: F, t206: F, t21747: F, t390: F, t5276: F, t5293: F, t5348: F, t5352: F, t5358: F, t5589: F, t5627: F, t5637: F, t5641: F, t5694: F, t5697: F, t5747: F, t5781: F, t5823: F, t664: F, t673: F, t681: F, t687: F, t689: F) -> (F, F, F) {
    let t22086 = 0.49911400759011601832e6 * t14 / t5670 / t175 * t5675 * t21066;
    let t22089 = 96.0 * t1907 * t645 * t5448;
    let t22096 = -0.79308603537392210532e2 * t390 * t5627 * t689 * t5589 + 0.1301229756036208781e0 * t390 * t5348 + 0.43374325201206959368e-1 * t390 * t5358 + 0.11558335953042377059e2 * t390 * t5352 + 0.45630383919063009625e3 * t390 * t5293 + 0.17005857815443677269e4 * t390 * t5747 * t206 * t2008 * t5589 + 0.41095999999999999999e0 * t390 * t673 * t2029 * t1932 + 0.39654301768696105267e2 * t390 * t1956 * t681 * t5641 + 0.88120670597102456145e1 * t1399 * t5637 + 0.1301229756036208781e0 * t390 * t5276 - 0.13674392837282271924e5 * t390 * t5694 * t206 * t5697 * t5589 + t22086 + t22089 - 0.77193501593724168324e3 * t5781 * t21747 * t664 + 72.0 * t687 * t681 * t5823;
    (t22086, t22089, t22096)
}
