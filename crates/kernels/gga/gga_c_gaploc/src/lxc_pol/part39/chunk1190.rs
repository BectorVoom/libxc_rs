//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1190/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1190<F: Float>(t40261: F, t38688: F, t895: F, t13814: F, t4953: F, t1445: F, t1562: F, t38613: F, t874: F, t40245: F, t41974: F, t41975: F, t41976: F, t41978: F, t41979: F, t41980: F, t41982: F, t41983: F) -> F {
    let t47994 = F::new(0.12780975317973583226e0) * t40261;
    let t47995 = t895 * t38688;
    let t47997 = t4953 * t13814;
    let t48001 = t1562 * t1445 * t38613 * t874;
    let t48003 = t41974 - t41975 - t41976 - F::new(0.76685851907841499354e0) * t40245 + t41978 + t41979 - t41980 - t47994 + F::new(0.23833659967900284446e0) * t47995 - F::new(0.69017266717057349418e1) * t47997 - F::new(0.69017266717057349418e1) * t48001 + t41982 - t41983;
    t48003
}
