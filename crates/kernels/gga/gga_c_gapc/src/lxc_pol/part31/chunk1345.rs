//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1345/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1345<F: Float>(t35806: F, t35809: F, t35811: F, t35815: F, t35820: F, t35823: F, t35826: F, t35829: F, t35831: F, t35835: F, t35838: F, t35841: F, t35843: F) -> F {
    let t36173 = F::cast_from(0.17399183805437348867e-6_f64) * t35806 - F::cast_from(0.46971924784082831588e-4_f64) * t35809 + F::cast_from(0.14580868318392378972e-3_f64) * t35811 + F::cast_from(0.38060714574394200647e-7_f64) * t35815 + F::cast_from(0.9785817663350589914e-7_f64) * t35820 + F::cast_from(0.46971924784082831588e-4_f64) * t35823 + F::cast_from(0.46971924784082831588e-4_f64) * t35826 + F::cast_from(0.23485962392041415794e-4_f64) * t35829 - F::cast_from(0.68394856556563412152e-6_f64) * t35831 - F::cast_from(0.22798285518854470718e-6_f64) * t35835 + F::cast_from(0.83516082266099274564e-5_f64) * t35838 + F::cast_from(0.29357452990051769742e-5_f64) * t35841 + F::cast_from(0.52892022403742372066e-4_f64) * t35843;
    t36173
}
