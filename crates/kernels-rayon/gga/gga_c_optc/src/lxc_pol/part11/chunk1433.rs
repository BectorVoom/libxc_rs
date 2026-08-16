//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1433/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1433(t1: f64, t4570: f64, t1162: f64, t15911: f64, t18099: f64, t26888: f64, t3234: f64, t3244: f64, t3245: f64, t4387: f64, t4435: f64, t4492: f64, t45418: f64, t45421: f64, t45424: f64, t45430: f64, t45439: f64, t46152: f64, t5378: f64, t5394: f64, t55039: f64, t55042: f64, t55044: f64, t58322: f64, t59715: f64, t59731: f64, t8537: f64, t914: f64) -> (f64, f64) {
    let t59822 = t4570 * t1;
    let t59835 = 0.31957282085435444036e5_f64 * t55039 - 0.23967961564076583027e5_f64 * t55042 + 0.1133330683113201024e1_f64 * t55044 + 0.2266661366226402048e1_f64 * t15911 * t5378 + 0.389869882632351453e2_f64 * t3234 * t4387 * t59731 + 0.10431793787746509425e1_f64 * t1162 * t914 * t8537 * t58322 + 0.90880810212048753088e1_f64 * t3244 * t3245 * t59715 + 0.15486228121497046737e3_f64 * t4435 * t46152 * t26888 * t59822 - 0.11721316454988582616e4_f64 * t45418 + 0.10324152080998031158e2_f64 * t45421 + 0.58606582274942913081e3_f64 * t45424 + 0.8317224162823497664e2_f64 * t45430 - 0.12475836244235246496e3_f64 * t4492 * t18099 + 0.51573792401949763511e5_f64 * t45439 * t5394;
    (t59822, t59835)
}
