//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 1037/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk1037<F: Float>(t34772: F, t34776: F, t34779: F, t34782: F, t34785: F, t34788: F, t34791: F, t34794: F, t34797: F, t34802: F, t34804: F, t11320: F, t11496: F, t628: F, t11499: F, t34372: F, t8621: F) -> (F, F, F, F) {
    let t34806 = 0.96684272530105650818e-8 * t34772 - 0.26194992237489957663e-9 * t34776 - 0.6956508833112845217e-4 * t34779 - 0.10020915386217878654e-6 * t34782 - 0.34752370105806885418e-3 * t34785 + 0.2504163411376437654e-5 * t34788 + 0.21720231316129303386e-4 * t34791 + 0.56863432614185654571e-5 * t34794 + 0.14732841528397051554e-8 * t34797 - 0.49166375783284505216e-8 * t34802 + 0.4637672555408563478e-4 * t34804;
    let t34808 = t628 * t11320 * t11496;
    let t34811 = t628 * t11499 * t11496;
    let t34813 = t34372 * t8621;
    (t34806, t34808, t34811, t34813)
}
