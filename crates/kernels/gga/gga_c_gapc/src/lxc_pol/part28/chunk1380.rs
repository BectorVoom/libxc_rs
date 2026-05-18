//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1380/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1380<F: Float>(t33660: F, t33671: F, t33674: F, t33682: F, t33690: F, t33694: F, t33697: F, t33680: F, t33687: F, t33692: F, t36659: F, t33701: F) -> (F, F) {
    let t36660 = F::new(0.50680539737635041234e-3) * t33660;
    let t36661 = F::new(0.52278590312710514777e-10) * t33671;
    let t36662 = F::new(0.1011909669415296852e-6) * t33674;
    let t36664 = F::new(0.2318836277704281739e-4) * t33682;
    let t36666 = F::new(0.14732367666458600006e-8) * t33690;
    let t36668 = F::new(0.18007519776492267795e-6) * t33694;
    let t36669 = F::new(0.43284943850479925795e-3) * t33697;
    let t36670 = -t36659 + t36660 - t36661 + t36662 - F::new(0.24457736545138888892e-4) * t33680 + t36664 - F::new(0.24457736545138888892e-4) * t33687 + t36666 + F::new(0.5691280480400994668e-7) * t33692 - t36668 + t36669;
    let t36671 = F::new(0.43440462632258606772e-4) * t33701;
    (t36670, t36671)
}
