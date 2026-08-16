//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1204/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1204<F: Float>(t11357: F, t27063: F, t34607: F, t5967: F, t1743: F, t20501: F, t33429: F, t11381: F, t9061: F, t33219: F, t5703: F, t11451: F, t11518: F, t1690: F) -> (F, F, F, F, F, F) {
    let t34870 = t11357 * t27063;
    let t34873 = t34607 * t5967;
    let t34876 = t1743 * t33429 * t20501;
    let t34878 = t9061 * t11381;
    let t34881 = t1743 * t33219 * t5703;
    let t34884 = t11518 * t11451 * t1690;
    (t34870, t34873, t34876, t34878, t34881, t34884)
}
