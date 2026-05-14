//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1093/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1093<F: Float>(t92876: F, t92932: F, t93149: F, t93201: F, t93250: F, t93299: F, t93345: F, t93393: F, t892: F, t11064: F, t7086: F, t1940: F, t1963: F, t1964: F, t2403: F, t25198: F, t25208: F, t25215: F, t25436: F, t25440: F, t25446: F, t25449: F, t25452: F, t30: F, t4541: F, t7010: F, t7087: F, t7091: F, t92795: F, t92799: F, t92806: F, t92810: F, t92814: F, t92819: F, t92822: F, t9344: F) -> (F, F, F, F) {
    let t93396 = t92876 + t92932 + t93149 + t93201 + t93250 + t93299 + t93345 + t93393;
    let t93397 = t93396 * t892;
    let t93404 = t7086 * t11064;
    let t93408 = 9.0 / 2.0 * t2403 * t1963 * t92795 + 9.0 / 2.0 * t2403 * t1963 * t92799 + 9.0 / 2.0 * t2403 * t25436 * t7010 + 9.0 * t4541 * t1963 * t92806 - t1940 * t7091 * t92810 / 2.0 + 3.0 / 2.0 * t2403 * t1963 * t92814 - 9.0 * t92819 * t25208 + 3.0 * t92822 * t1964 + 9.0 * t4541 * t7087 * t25198 - 3.0 / 2.0 * t1940 * t25440 * t25452 + 9.0 / 2.0 * t2403 * t7087 * t25215 + t1940 * t1963 * t9344 / 2.0 + t1940 * t93397 * t30 / 2.0 - 3.0 * t1940 * t25440 * t25449 + 3.0 * t1940 * t93404 * t25446;
    (t93396, t93397, t93404, t93408)
}
