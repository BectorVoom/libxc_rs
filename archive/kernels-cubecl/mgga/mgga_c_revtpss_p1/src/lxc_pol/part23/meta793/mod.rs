//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta793 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2611;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2612;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta793<F: Float>(t5977: F, t836: F, t10811: F, t18462: F, t18466: F, t125: F, t18615: F, t10744: F, t18418: F, t808: F, t18446: F, t10886: F, t18599: F, t1544: F, t1559: F, t40834: F, t854: F, t18413: F, t18525: F, t2661: F, t40693: F, t10726: F, t4366: F, t18608: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t61756, t61774, t61776, t61791, t61797, t61817, t61833) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2611::<F>(t5977, t836, t10811, t18462, t18466, t125, t18615, t10744, t18418, t808, t18446, t10886, t18599);
        let (t61837, t61839, t61860, t61864, t61877) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2612::<F>(t1544, t1559, t40834, t854, t18413, t18525, t2661, t40693, t10726, t4366, t10886, t18608, t808);
    (t61756, t61774, t61776, t61791, t61797, t61817, t61833, t61837, t61839, t61860, t61864, t61877)
}
