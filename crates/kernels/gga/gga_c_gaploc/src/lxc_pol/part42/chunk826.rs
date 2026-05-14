//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 826/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk826<F: Float>(t12012: F, t1358: F, t14277: F, t14280: F, t2783: F, t3394: F, t44443: F, t44457: F, t44469: F, t44473: F, t44477: F, t44479: F, t44483: F, t44485: F, t44487: F, t44489: F, t44490: F, t44491: F, t44493: F, t47009: F, t488: F, t6305: F, t6313: F) -> (F,) {
    let t49907 = -t44443 - t44457 - t44469 + t44473 - t44477 + t44479 + t44483 + t44485 + t44487 - t44489 + t44490 + t44491 - t44493 - 0.2276400530854091968e0 * t6313 * t14280 + 0.7588001769513639893e-1 * t6313 * t14277 - 0.1707300398140568976e0 * t6305 * t14280 - 0.63233348079280332442e-2 * t1358 * t12012 * t3394 * t488 + 0.18970004423784099733e-1 * t1358 * t47009 * t2783;
    (t49907,)
}
