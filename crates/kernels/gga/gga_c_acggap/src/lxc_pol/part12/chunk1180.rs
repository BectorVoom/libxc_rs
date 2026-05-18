//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1180/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1180<F: Float>(t34794: F, t34804: F, t30730: F, t30738: F, t30744: F, t30748: F, t30750: F, t30756: F, t30758: F, t30763: F, t30767: F, t34798: F, t34802: F, t34817: F, t34821: F, t34826: F, t34830: F) -> F {
    let t37249 = F::new(0.31448092289604152068e-2) * t34794;
    let t37252 = F::new(0.20965394859736101378e-2) * t34804;
    let t37266 = t37249 + F::new(0.15724046144802076034e-2) * t34798 + F::new(0.20965394859736101378e-2) * t34802 + t37252 - F::new(0.12579236915841660828e-2) * t30730 - F::new(0.62896184579208304137e-2) * t30738 - F::new(0.18868855373762491241e-2) * t30744 + F::new(0.83861579438944405516e-3) * t30748 - F::new(0.34299214494455789578e-2) * t30750 + F::new(0.34299214494455789578e-2) * t30756 + F::new(0.12579236915841660828e-2) * t30758 + F::new(7.0) / F::new(36.0) * t30763 + F::new(7.0) / F::new(72.0) * t30767 - F::new(0.62896184579208304138e-2) * t34817 - F::new(0.37737710747524982482e-2) * t34821 - F::new(0.25158473831683321656e-2) * t34826 + F::new(0.37737710747524982482e-2) * t34830;
    t37266
}
