//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 875/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk875<F: Float>(t31542: F, t31597: F, t31601: F, t31662: F, t31720: F, t31750: F, t31805: F, t31839: F, t31867: F, t2138: F, t2147: F, t463: F, t8064: F, t1265: F, t8331: F, t2132: F, t3037: F, t32146: F, t633: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t32850 = 0.62896184579208304137e-3 * t31542;
    let t32866 = 0.21437009059034868486e-3 * t31597;
    let t32867 = 0.42874018118069736972e-3 * t31601;
    let t32891 = 0.77173232612525526551e-2 * t31662;
    let t32915 = 0.18868855373762491242e-2 * t31720;
    let t32923 = 0.27010631414383934293e-1 * t31750;
    let t32942 = 0.12862205435420921092e-2 * t31805;
    let t32955 = 0.85748036236139473944e-3 * t31839;
    let t32967 = 0.2767432121485165382e-1 * t31867;
    let t32990 = t2138 * t2147 * t8064 * t463;
    let t32992 = t8331 * t1265;
    let t32997 = 0.10408353825846239354e2 * t32146 * t2132 * t633 * t3037;
    (t32850, t32866, t32867, t32891, t32915, t32923, t32942, t32955, t32967, t32990, t32992, t32997)
}
