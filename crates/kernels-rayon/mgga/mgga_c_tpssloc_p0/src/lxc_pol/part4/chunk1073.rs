//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1073/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1073(t17783: f64, t973: f64, t13861: f64, t4531: f64, t17178: f64, t4510: f64, t2989: f64, t5398: f64, t2988: f64, t10186: f64, t13830: f64, t13850: f64, t17770: f64, t17773: f64, t17778: f64, t2960: f64, t2986: f64, t5818: f64, t5821: f64, t5829: f64) -> f64 {
    let t17784 = t973 * t17783;
    let t17788 = t4531 * t13861;
    let t17791 = t4510 * t17178;
    let t17794 = t2989 * t5398;
    let t17795 = t2988 * t17794;
    let t17798 = t13830 - 0.74074074074074074072e-3_f64 * t2960 * t5829 + 0.9259259259259259259e-4_f64 * t17770 + 0.27777777777777777777e-3_f64 * t973 * t17773 - 0.83333333333333333332e-3_f64 * t973 * t17778 - 0.98765432098765432096e-3_f64 * t2960 * t5818 + 0.12345679012345679012e-3_f64 * t17784 + 0.14814814814814814814e-2_f64 * t10186 * t5821 - 0.55555555555555555554e-3_f64 * t2986 * t17788 - t13850 + 0.37037037037037037036e-3_f64 * t2986 * t17791 - 0.27777777777777777777e-3_f64 * t2986 * t17795;
    t17798
}
