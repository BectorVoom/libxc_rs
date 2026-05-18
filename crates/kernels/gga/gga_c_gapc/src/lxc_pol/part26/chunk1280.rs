//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1280/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1280<F: Float>(t11632: F, t2245: F, t6201: F, t35759: F, t35762: F, t35764: F, t35768: F, t35772: F, t35776: F, t35780: F, t35783: F, t35788: F, t35792: F, t35795: F, t35798: F) -> F {
    let t35801 = t11632 * t2245 * t6201;
    let t35803 = -F::new(0.17098714139140853038e-6) * t35759 - F::new(0.99742499144988309388e-7) * t35762 + F::new(0.16146599144528473358e-4) * t35764 - F::new(0.10703238069289707718e-7) * t35768 - F::new(0.34197428278281706076e-6) * t35772 - F::new(0.86995919027186744338e-7) * t35776 - F::new(0.86995919027186744338e-7) * t35780 - F::new(0.14678726495025884871e-5) * t35783 - F::new(0.24748599044854085031e-6) * t35788 - F::new(0.14678726495025884871e-5) * t35792 - F::new(0.14678726495025884871e-5) * t35795 - F::new(0.73393632475129424356e-6) * t35798 + F::new(0.86995919027186744338e-7) * t35801;
    t35803
}
