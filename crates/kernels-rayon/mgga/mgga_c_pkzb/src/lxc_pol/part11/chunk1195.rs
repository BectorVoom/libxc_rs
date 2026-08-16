//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1195/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1195(t1031: f64, t10604: f64, t133: f64, t158: f64, t162: f64, t2625: f64, t2633: f64, t2636: f64, t29111: f64, t29112: f64, t29114: f64, t29115: f64, t29125: f64, t29129: f64, t29138: f64, t29152: f64, t29209: f64, t3431: f64, t3435: f64, t3438: f64, t597: f64, t8859: f64, t8865: f64, t8873: f64, t8876: f64, t8882: f64) -> f64 {
    let t29210 = -(t29111 + t29112 + t29114 + t29115 + t29125 + t29129 + t29138 + t29152) * t158 * t162 + 3.0_f64 * t10604 * t597 + 9.0_f64 * t8859 * t1031 - 36.0_f64 * t3431 * t133 * t2633 + 9.0_f64 * t3431 * t2636 - 36.0_f64 * t2625 * t3435 + 180.0_f64 * t8865 * t8873 - 72.0_f64 * t8865 * t8876 + 9.0_f64 * t2625 * t3438 - 36.0_f64 * t8865 * t8882 + t29209;
    t29210
}
