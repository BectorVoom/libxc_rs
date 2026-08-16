//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 957/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk957(t12012: f64, t1358: f64, t14277: f64, t14280: f64, t2783: f64, t3394: f64, t44443: f64, t44457: f64, t44469: f64, t44473: f64, t44477: f64, t44479: f64, t44483: f64, t44485: f64, t44487: f64, t44489: f64, t44490: f64, t44491: f64, t44493: f64, t47009: f64, t488: f64, t6305: f64, t6313: f64) -> f64 {
    let t49907 = -t44443 - t44457 - t44469 + t44473 - t44477 + t44479 + t44483 + t44485 + t44487 - t44489 + t44490 + t44491 - t44493 - 0.2276400530854091968e0_f64 * t6313 * t14280 + 0.7588001769513639893e-1_f64 * t6313 * t14277 - 0.1707300398140568976e0_f64 * t6305 * t14280 - 0.63233348079280332442e-2_f64 * t1358 * t12012 * t3394 * t488 + 0.18970004423784099733e-1_f64 * t1358 * t47009 * t2783;
    t49907
}
