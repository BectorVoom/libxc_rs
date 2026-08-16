//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta580 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1960;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1961;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1962;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta580(t1011: f64, t6224: f64, t3508: f64, t24661: f64, t475: f64, t24668: f64, t2132: f64, t28525: f64, t1726: f64, t2136: f64, t24659: f64, t27674: f64, t27677: f64, t27681: f64, t27701: f64, t6178: f64, t6184: f64, t6188: f64, t6207: f64, t7310: f64, t7345: f64, t29580: f64, t29610: f64, t29636: f64, t466: f64, t1238: f64, t1761: f64, t27406: f64, t27792: f64, t29532: f64, t29536: f64, t29546: f64, t29551: f64, t29554: f64, t29557: f64, t498: f64, t5055: f64, t6244: f64, t7283: f64, t7351: f64, t8003: f64, t8061: f64, t225: f64, t497: f64, t6238: f64, t462: f64, t27751: f64, t8014: f64, t1887: f64, t29584: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t29643, t29644, t29647, t29648, t29651, t29662) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1960(t1011, t6224, t3508, t24661, t475, t24668, t2132, t28525, t1726, t2136, t24659, t27674, t27677, t27681, t27701, t6178, t6184, t6188, t6207, t7310, t7345);
        let (t29664, t29665, t29667) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1961(t29580, t29610, t29636, t29662, t466, t1238, t1761, t27406, t27792, t29532, t29536, t29546, t29551, t29554, t29557, t498, t5055, t6244, t7283, t7351, t8003, t8061);
        let (t29670, t29671, t29674, t29678) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1962(t225, t497, t6238, t462, t27751, t8014, t1887, t29584);
    (t29643, t29644, t29647, t29648, t29651, t29664, t29665, t29667, t29670, t29671, t29674, t29678)
}
