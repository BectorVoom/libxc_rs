//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1040/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1040<F: Float>(t5148: F, t551: F, t71910: F, t14444: F, t1587: F, t76255: F, t76258: F, t76262: F, t3203: F, t570: F, t1614: F, t5266: F) -> (F, F, F, F, F, F, F) {
    let t77992 = F::cast_from(0.11974241701863808564e0_f64) * t5148 * t71910 * t551;
    let t77995 = F::cast_from(0.11974241701863808564e0_f64) * t5148 * t14444 * t1587;
    let t77996 = F::cast_from(0.81823984962736025192e-1_f64) * t76255;
    let t77997 = F::cast_from(0.40911992481368012596e-1_f64) * t76258;
    let t77998 = F::cast_from(0.8182398496273602519e-1_f64) * t76262;
    let t77999 = t3203 * t570;
    let t78005 = F::cast_from(0.11974241701863808564e0_f64) * t5266 * t14444 * t1614;
    (t77992, t77995, t77996, t77997, t77998, t77999, t78005)
}
