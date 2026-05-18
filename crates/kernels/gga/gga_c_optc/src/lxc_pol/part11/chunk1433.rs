//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1433/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1433<F: Float>(t1: F, t4570: F, t1162: F, t15911: F, t18099: F, t26888: F, t3234: F, t3244: F, t3245: F, t4387: F, t4435: F, t4492: F, t45418: F, t45421: F, t45424: F, t45430: F, t45439: F, t46152: F, t5378: F, t5394: F, t55039: F, t55042: F, t55044: F, t58322: F, t59715: F, t59731: F, t8537: F, t914: F) -> (F, F) {
    let t59822 = t4570 * t1;
    let t59835 = F::new(0.31957282085435444036e5) * t55039 - F::new(0.23967961564076583027e5) * t55042 + F::new(0.1133330683113201024e1) * t55044 + F::new(0.2266661366226402048e1) * t15911 * t5378 + F::new(0.389869882632351453e2) * t3234 * t4387 * t59731 + F::new(0.10431793787746509425e1) * t1162 * t914 * t8537 * t58322 + F::new(0.90880810212048753088e1) * t3244 * t3245 * t59715 + F::new(0.15486228121497046737e3) * t4435 * t46152 * t26888 * t59822 - F::new(0.11721316454988582616e4) * t45418 + F::new(0.10324152080998031158e2) * t45421 + F::new(0.58606582274942913081e3) * t45424 + F::new(0.8317224162823497664e2) * t45430 - F::new(0.12475836244235246496e3) * t4492 * t18099 + F::new(0.51573792401949763511e5) * t45439 * t5394;
    (t59822, t59835)
}
