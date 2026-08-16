//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 995/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk995<F: Float>(t33643: F, t944: F, t157: F, t556: F, t929: F, t5299: F, t615: F, t406: F, t463: F, t7884: F, t8396: F, t2137: F, t32123: F) -> (F, F, F, F, F, F) {
    let t33644 = t33643 * t944;
    let t33651 = t556 * t929 * t157;
    let t33658 = t615 * t5299;
    let t33675 = t944 * t463 * t406;
    let t33682 = t7884 * t8396;
    let t33698 = t2137 * t32123;
    (t33644, t33651, t33658, t33675, t33682, t33698)
}
