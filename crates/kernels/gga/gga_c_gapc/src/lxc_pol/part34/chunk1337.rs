//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1337/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1337<F: Float>(t35759: F, t35762: F, t35764: F, t35768: F, t35772: F, t35776: F, t35780: F, t35783: F, t35788: F, t35792: F, t35795: F, t35798: F, t35801: F) -> F {
    let t36158 = -F::cast_from(0.34197428278281706076e-6_f64) * t35759 - F::cast_from(0.19948499828997661878e-6_f64) * t35762 + F::cast_from(0.32293198289056946716e-4_f64) * t35764 - F::cast_from(0.21406476138579415437e-7_f64) * t35768 - F::cast_from(0.68394856556563412152e-6_f64) * t35772 - F::cast_from(0.17399183805437348867e-6_f64) * t35776 - F::cast_from(0.17399183805437348867e-6_f64) * t35780 - F::cast_from(0.29357452990051769742e-5_f64) * t35783 - F::cast_from(0.49497198089708170061e-6_f64) * t35788 - F::cast_from(0.29357452990051769742e-5_f64) * t35792 - F::cast_from(0.29357452990051769742e-5_f64) * t35795 - F::cast_from(0.14678726495025884871e-5_f64) * t35798 + F::cast_from(0.17399183805437348867e-6_f64) * t35801;
    t36158
}
