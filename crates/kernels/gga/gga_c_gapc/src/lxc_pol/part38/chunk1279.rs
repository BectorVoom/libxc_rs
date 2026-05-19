//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1279/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1279<F: Float>(t11632: F, t2245: F, t6201: F, t35759: F, t35762: F, t35764: F, t35768: F, t35772: F, t35776: F, t35780: F, t35783: F, t35788: F, t35792: F, t35795: F, t35798: F) -> F {
    let t35801 = t11632 * t2245 * t6201;
    let t35803 = -F::cast_from(0.17098714139140853038e-6_f64) * t35759 - F::cast_from(0.99742499144988309388e-7_f64) * t35762 + F::cast_from(0.16146599144528473358e-4_f64) * t35764 - F::cast_from(0.10703238069289707718e-7_f64) * t35768 - F::cast_from(0.34197428278281706076e-6_f64) * t35772 - F::cast_from(0.86995919027186744338e-7_f64) * t35776 - F::cast_from(0.86995919027186744338e-7_f64) * t35780 - F::cast_from(0.14678726495025884871e-5_f64) * t35783 - F::cast_from(0.24748599044854085031e-6_f64) * t35788 - F::cast_from(0.14678726495025884871e-5_f64) * t35792 - F::cast_from(0.14678726495025884871e-5_f64) * t35795 - F::cast_from(0.73393632475129424356e-6_f64) * t35798 + F::cast_from(0.86995919027186744338e-7_f64) * t35801;
    t35803
}
