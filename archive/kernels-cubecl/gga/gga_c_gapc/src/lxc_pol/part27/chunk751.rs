//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 751/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk751<F: Float>(t1030: F, t8686: F, t1040: F, t2974: F, t3064: F, t2973: F, t2979: F, t5987: F, t2983: F, t1: F, t118: F, t3: F) -> (F, F, F, F, F) {
    let t8798 = t1030 * t8686;
    let t8799 = t8798 * t1040;
    let t8801 = t3064 * t2974;
    let t8802 = t2973 * t8801;
    let t8804 = t5987 * t2979;
    let t8805 = t8804 * t2983;
    let t8808 = t118 * t1 * t3;
    (t8798, t8799, t8802, t8805, t8808)
}
