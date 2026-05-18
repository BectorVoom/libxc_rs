//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1177/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1177<F: Float>(t34659: F, t30661: F, t30673: F, t30675: F, t30690: F, t30695: F, t30705: F, t30709: F, t32543: F, t32544: F, t32545: F, t34657: F, t34663: F, t34667: F, t34671: F, t34675: F, t34684: F, t34686: F) -> F {
    let t37197 = F::new(7.0) / F::new(36.0) * t34659;
    let t37208 = F::new(0.80031500487063509014e-2) * t30661 - t32543 - t32544 + t32545 - F::new(0.68598428988911579156e-2) * t30673 - F::new(0.34299214494455789578e-2) * t30675 - t34657 / F::new(48.0) + t37197 + F::new(0.62896184579208304138e-3) * t34663 + F::new(0.12862205435420921092e-2) * t34667 + F::new(0.94344276868812456204e-2) * t34671 + F::new(0.83861579438944405518e-3) * t34675 - F::new(0.68598428988911579156e-2) * t30690 + F::new(0.14291339372689912324e-2) * t30695 - F::new(0.2096539485973610138e-2) * t30705 - F::new(0.12579236915841660827e-2) * t30709 - F::new(0.12862205435420921092e-2) * t34684 - t34686 / F::new(24.0);
    t37208
}
