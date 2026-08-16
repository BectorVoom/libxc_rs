//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta433 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1667;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta433<F: Float>(t16306: F, t6394: F, t16305: F, t16225: F, t16311: F, t1825: F, t5308: F, t16224: F, t12286: F, t1341: F, t16239: F, t16241: F, t16269: F, t16290: F, t16294: F, t16317: F, t16325: F, t16331: F, t16338: F, t16341: F, t19868: F, t19873: F, t19876: F, t19879: F, t19882: F, t3778: F, t3803: F, t5246: F, t5252: F, t6390: F, t6417: F) -> (F, F, F, F, F, F, F) {
        let (t19885, t19886, t19889, t19890, t19893, t19894, t19899) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1667::<F>(t16306, t6394, t16305, t16225, t16311, t1825, t5308, t16224, t12286, t1341, t16239, t16241, t16269, t16290, t16294, t16317, t16325, t16331, t16338, t16341, t19868, t19873, t19876, t19879, t19882, t3778, t3803, t5246, t5252, t6390, t6417);
    (t19885, t19886, t19889, t19890, t19893, t19894, t19899)
}
