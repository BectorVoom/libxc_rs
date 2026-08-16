//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1178/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1178<F: Float>(t52670: F, t65850: F, t65853: F, t65860: F, t65862: F, t88575: F, t88577: F, t88579: F, t88581: F, t88585: F, t88596: F, t43631: F, t79786: F, t79789: F, t79792: F, t79794: F, t79796: F, t79799: F, t79809: F, t88621: F, t88624: F, t88626: F) -> (F, F) {
    let t90124 = -F::cast_from(0.10668480174814814815e1_f64) * t88575 - F::cast_from(0.30424924942990397807e1_f64) * t88577 + F::cast_from(0.97794401602469135807e0_f64) * t88579 + F::cast_from(0.13039253546995884774e1_f64) * t88581 - F::cast_from(0.62232801019753086422e0_f64) * t88585 - F::cast_from(0.22226000364197530866e-1_f64) * t65850 - F::cast_from(0.29634667152263374488e-1_f64) * t65853 + F::cast_from(0.44452000728395061732e-1_f64) * t65860 + F::cast_from(0.11853866860905349795e0_f64) * t65862 - F::cast_from(0.19756444768175582992e0_f64) * t52670 + F::cast_from(0.12930593100770919068e2_f64) * t88596;
    let t90147 = F::cast_from(0.8890400145679012346e-1_f64) * t88621 - F::cast_from(0.80013601311111111114e0_f64) * t88624 - t43631 + F::cast_from(0.10668480174814814815e1_f64) * t88626 - F::cast_from(0.2370773372181069959e0_f64) * t79786 + F::cast_from(0.69147556688614540471e-1_f64) * t79789 + F::cast_from(0.1333560021851851852e0_f64) * t79792 + F::cast_from(0.65196267734979423872e0_f64) * t79794 - F::cast_from(0.17780800291358024692e0_f64) * t79796 - F::cast_from(0.17780800291358024693e0_f64) * t79799 - F::cast_from(0.30424924942990397807e1_f64) * t79809;
    (t90124, t90147)
}
