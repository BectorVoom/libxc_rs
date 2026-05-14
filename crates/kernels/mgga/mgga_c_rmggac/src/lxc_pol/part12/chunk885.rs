//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 885/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk885<F: Float>(t38745: F, t5271: F, t39670: F, t5162: F, t39674: F, t4669: F, t305: F, t38674: F, t118: F, t25809: F, t39692: F, t2123: F, t558: F, t321: F, t35918: F, t35922: F, t35926: F, t35937: F, t5259: F, t833: F, t848: F, t876: F, t8936: F, t8975: F) -> (F, F) {
    let t41101 = t5271 * t38745;
    let t41106 = t5162 * t39670;
    let t41108 = t4669 * t39674;
    let t41114 = t305 * t38674;
    let t41115 = 0.79828278012425390426e-1 * t41114;
    let t41116 = t118 * t25809;
    let t41120 = t5271 * t39692;
    let t41122 = t2123 * t558;
    let t41126 = -0.17961362552795712846e0 * t4669 * t8975 * t848 - 0.17961362552795712846e0 * t41101 + 0.11974241701863808564e0 * t5259 * t8975 * t833 + 0.35922725105591425692e0 * t41106 + 0.8980681276397856423e-1 * t41108 + 0.47896966807455234256e0 * t35918 + 0.66671395154821946448e-1 * t35922 + 0.2666855806192877858e0 * t35926 + 0.18183107769496894486e-1 * t35937 + t41115 - 0.47896966807455234256e0 * t41116 * t8936 * t876 + 0.47896966807455234256e0 * t41120 - 0.35922725105591425692e0 * t4669 * t41122 * t321;
    (t41122, t41126)
}
