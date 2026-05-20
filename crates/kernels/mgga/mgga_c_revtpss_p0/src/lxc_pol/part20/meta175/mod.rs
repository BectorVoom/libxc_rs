//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta175 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk914;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk915;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta175<F: Float>(t9570: F, t2626: F, t676: F, t3869: F, t2434: F, t762: F, t1331: F, t3860: F, t1320: F, t3855: F, t186: F, t685: F, t793: F, t1337: F, t4135: F, t5541: F, t7315: F, t9514: F, t9517: F, t9521: F, t9560: F, t9562: F, t9565: F, t9567: F, t9569: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t9571, t9572, t9574, t9575, t9577, t9579, t9581, t9586) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk914::<F>(t9570, t2626, t676, t3869, t2434, t762, t1331, t3860, t1320, t3855, t186, t685, t793);
        let (t9588, t9589) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk915::<F>(t1337, t9586, t4135, t5541, t7315, t9514, t9517, t9521, t9560, t9562, t9565, t9567, t9569, t9571, t9574, t9577, t9579, t9581);
    (t9571, t9572, t9574, t9575, t9577, t9579, t9581, t9586, t9588, t9589)
}
