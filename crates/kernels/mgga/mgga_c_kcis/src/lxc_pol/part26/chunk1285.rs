//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1285/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1285<F: Float>(t1394: F, t7100: F, t94216: F, t22285: F, t27387: F, t22290: F, t5780: F, t101853: F, t101994: F, t101997: F, t102001: F, t102005: F, t28708: F, t28727: F, t28844: F, t28853: F, t7978: F, t7981: F, t98162: F) -> (F, F, F, F) {
    let t102011 = t1394 * t94216 * t7100;
    let t102014 = t1394 * t27387 * t22285;
    let t102017 = t5780 * t27387 * t22290;
    let t102025 = -F::new(0.51588271604938271603e-3) * t98162 + F::new(0.61905925925925925925e-2) * t101994 + F::new(0.20635308641975308642e-2) * t101997 + F::new(0.69644166666666666664e-2) * t102001 - F::new(0.69505208333333333334e-3) * t7978 * t102005 + F::new(0.61782407407407407407e-3) * t101853 * t7981 - F::new(0.23214722222222222222e-2) * t102011 - F::new(0.23214722222222222222e-2) * t102014 + F::new(0.46429444444444444444e-2) * t102017 + F::new(0.74203760416666666667e-3) * t28853 * t28708 + F::new(0.37069444444444444444e-2) * t28727 * t28708 - F::new(0.12356481481481481482e-2) * t28727 * t28844;
    (t102011, t102014, t102017, t102025)
}
