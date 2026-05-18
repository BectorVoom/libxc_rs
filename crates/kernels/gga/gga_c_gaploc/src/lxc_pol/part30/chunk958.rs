//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 958/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk958<F: Float>(t10442: F, t10430: F, t912: F, t587: F, t2293: F, t2854: F, t1445: F, t1562: F, t10151: F, t447: F, t3354: F, t4673: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10443 = F::new(0.19171462976960374838e0) * t10442;
    let t10444 = t912 * t10430;
    let t10445 = t587 * t10444;
    let t10446 = F::new(0.19171462976960374838e0) * t10445;
    let t10447 = t2854 * t2293;
    let t10448 = t1445 * t10447;
    let t10450 = F::new(0.69017266717057349418e1) * t1562 * t10448;
    let t10451 = t10151 * t447;
    let t10452 = t1445 * t10451;
    let t10455 = t4673 * t3354;
    (t10443, t10444, t10446, t10447, t10448, t10450, t10451, t10452, t10455)
}
