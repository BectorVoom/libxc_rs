//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1086/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1086<F: Float>(t1031: F, t10604: F, t133: F, t158: F, t162: F, t2625: F, t2633: F, t2636: F, t29111: F, t29112: F, t29114: F, t29115: F, t29125: F, t29129: F, t29138: F, t29152: F, t29209: F, t3431: F, t3435: F, t3438: F, t597: F, t8859: F, t8865: F, t8873: F, t8876: F, t8882: F) -> (F,) {
    let t29210 = -(t29111 + t29112 + t29114 + t29115 + t29125 + t29129 + t29138 + t29152) * t158 * t162 + 3.0 * t10604 * t597 + 9.0 * t8859 * t1031 - 36.0 * t3431 * t133 * t2633 + 9.0 * t3431 * t2636 - 36.0 * t2625 * t3435 + 180.0 * t8865 * t8873 - 72.0 * t8865 * t8876 + 9.0 * t2625 * t3438 - 36.0 * t8865 * t8882 + t29209;
    (t29210,)
}
