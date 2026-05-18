//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1338/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1338<F: Float>(t35806: F, t35809: F, t35811: F, t35815: F, t35820: F, t35823: F, t35826: F, t35829: F, t35831: F, t35835: F, t35838: F, t35841: F, t35843: F) -> F {
    let t36173 = F::new(0.17399183805437348867e-6) * t35806 - F::new(0.46971924784082831588e-4) * t35809 + F::new(0.14580868318392378972e-3) * t35811 + F::new(0.38060714574394200647e-7) * t35815 + F::new(0.9785817663350589914e-7) * t35820 + F::new(0.46971924784082831588e-4) * t35823 + F::new(0.46971924784082831588e-4) * t35826 + F::new(0.23485962392041415794e-4) * t35829 - F::new(0.68394856556563412152e-6) * t35831 - F::new(0.22798285518854470718e-6) * t35835 + F::new(0.83516082266099274564e-5) * t35838 + F::new(0.29357452990051769742e-5) * t35841 + F::new(0.52892022403742372066e-4) * t35843;
    t36173
}
