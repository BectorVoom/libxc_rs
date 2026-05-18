//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 940/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk940<F: Float>(t31494: F, t31498: F, t31508: F, t31524: F, t31542: F, t31597: F, t31601: F, t31662: F, t31720: F, t31750: F, t31805: F, t31839: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t32833 = F::new(0.56606566121287473723e-2) * t31494;
    let t32834 = F::new(0.12579236915841660828e-2) * t31498;
    let t32839 = F::new(0.10561041666666666667e1) * t31508;
    let t32844 = F::new(0.18868855373762491241e-2) * t31524;
    let t32850 = F::new(0.62896184579208304137e-3) * t31542;
    let t32866 = F::new(0.21437009059034868486e-3) * t31597;
    let t32867 = F::new(0.42874018118069736972e-3) * t31601;
    let t32891 = F::new(0.77173232612525526551e-2) * t31662;
    let t32915 = F::new(0.18868855373762491242e-2) * t31720;
    let t32923 = F::new(0.27010631414383934293e-1) * t31750;
    let t32942 = F::new(0.12862205435420921092e-2) * t31805;
    let t32955 = F::new(0.85748036236139473944e-3) * t31839;
    (t32833, t32834, t32839, t32844, t32850, t32866, t32867, t32891, t32915, t32923, t32942, t32955)
}
