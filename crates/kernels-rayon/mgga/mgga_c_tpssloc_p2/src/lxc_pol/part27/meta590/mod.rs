//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta590 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2048;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2049;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta590(t131: f64, t23121: f64, t9537: f64, t236: f64, t81613: f64, t23098: f64, t22822: f64, t281: f64, t6589: f64, t23124: f64, t23076: f64, t6597: f64, t22690: f64, t2379: f64, t841: f64, t23072: f64, t23083: f64, t23069: f64, t2610: f64, t2690: f64, t6612: f64, t812: f64, t831: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t81782, t81783, t81785, t81788, t81789, t81792) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2048(t131, t23121, t9537, t236, t81613, t23098, t22822, t281, t6589, t23124, t23076, t6597);
        let (t81795, t81797, t81799, t81807, t81808) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2049(t22690, t2379, t81792, t841, t23072, t23083, t23069, t2610, t2690, t6612, t812, t831);
    (t81782, t81783, t81785, t81788, t81789, t81792, t81795, t81797, t81799, t81807, t81808)
}
