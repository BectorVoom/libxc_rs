//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1195/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1195<F: Float>(t27149: F, t520: F, t9061: F, t11449: F, t11451: F, t1803: F, t190: F, t21183: F, t11492: F, t34675: F, t34772: F, t34776: F, t34779: F, t34782: F, t34785: F, t34788: F, t34791: F, t34794: F) -> F {
    let t34797 = t9061 * t520 * t27149;
    let t34802 = t1803 * t190 * t11449 * t11451 * t21183;
    let t34804 = t34675 * t11492;
    let t34806 = F::new(0.96684272530105650818e-8) * t34772 - F::new(0.26194992237489957663e-9) * t34776 - F::new(0.6956508833112845217e-4) * t34779 - F::new(0.10020915386217878654e-6) * t34782 - F::new(0.34752370105806885418e-3) * t34785 + F::new(0.2504163411376437654e-5) * t34788 + F::new(0.21720231316129303386e-4) * t34791 + F::new(0.56863432614185654571e-5) * t34794 + F::new(0.14732841528397051554e-8) * t34797 - F::new(0.49166375783284505216e-8) * t34802 + F::new(0.4637672555408563478e-4) * t34804;
    t34806
}
