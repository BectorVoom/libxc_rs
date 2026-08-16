//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta580 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1960;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1961;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1962;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta580<F: Float>(t1011: F, t6224: F, t3508: F, t24661: F, t475: F, t24668: F, t2132: F, t28525: F, t1726: F, t2136: F, t24659: F, t27674: F, t27677: F, t27681: F, t27701: F, t6178: F, t6184: F, t6188: F, t6207: F, t7310: F, t7345: F, t29580: F, t29610: F, t29636: F, t466: F, t1238: F, t1761: F, t27406: F, t27792: F, t29532: F, t29536: F, t29546: F, t29551: F, t29554: F, t29557: F, t498: F, t5055: F, t6244: F, t7283: F, t7351: F, t8003: F, t8061: F, t225: F, t497: F, t6238: F, t462: F, t27751: F, t8014: F, t1887: F, t29584: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t29643, t29644, t29647, t29648, t29651, t29662) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1960::<F>(t1011, t6224, t3508, t24661, t475, t24668, t2132, t28525, t1726, t2136, t24659, t27674, t27677, t27681, t27701, t6178, t6184, t6188, t6207, t7310, t7345);
        let (t29664, t29665, t29667) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1961::<F>(t29580, t29610, t29636, t29662, t466, t1238, t1761, t27406, t27792, t29532, t29536, t29546, t29551, t29554, t29557, t498, t5055, t6244, t7283, t7351, t8003, t8061);
        let (t29670, t29671, t29674, t29678) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1962::<F>(t225, t497, t6238, t462, t27751, t8014, t1887, t29584);
    (t29643, t29644, t29647, t29648, t29651, t29664, t29665, t29667, t29670, t29671, t29674, t29678)
}
