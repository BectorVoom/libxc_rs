//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1276/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1276<F: Float>(t35720: F, t35722: F, t35725: F, t35727: F, t35732: F, t35736: F, t35738: F, t35741: F, t35745: F, t35747: F, t35749: F, t35753: F, t35755: F) -> F {
    let t35757 = F::new(0.14678726495025884871e-5) * t35720 - F::new(0.23485962392041415794e-4) * t35722 - F::new(0.73393632475129424356e-6) * t35725 + F::new(0.17098714139140853038e-6) * t35727 + F::new(0.99742499144988309388e-7) * t35732 + F::new(0.34197428278281706076e-6) * t35736 + F::new(0.17098714139140853038e-6) * t35738 - F::new(0.4892908831675294957e-7) * t35741 + F::new(0.64219428415738246312e-6) * t35745 - F::new(0.1352698274118698596e-4) * t35747 + F::new(0.3077768545045353547e-5) * t35749 + F::new(0.41758041133049637282e-5) * t35753 + F::new(0.94755374187738893921e-6) * t35755;
    t35757
}
