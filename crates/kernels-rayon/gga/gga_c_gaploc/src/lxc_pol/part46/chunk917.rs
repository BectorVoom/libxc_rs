//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 917/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk917(t42846: f64, t31586: f64, t4261: f64, t9074: f64, t1358: f64, t42433: f64, t6507: f64, t1063: f64, t2854: f64, t29969: f64, t6320: f64, t1064: f64, t42086: f64, t42821: f64, t42822: f64, t42823: f64, t42824: f64, t42826: f64, t42828: f64, t42829: f64, t42832: f64, t42835: f64, t42838: f64, t42841: f64, t42844: f64, t42845: f64) -> f64 {
    let t42847 = 0.47425011059460249332e-2_f64 * t42846;
    let t42849 = t9074 * t4261 * t31586;
    let t42850 = 0.47425011059460249332e-2_f64 * t42849;
    let t42852 = t1358 * t6507 * t42433;
    let t42857 = 0.17073003981405689759e0_f64 * t1063 * t6320 * t2854 * t29969;
    let t42861 = -t42821 - t42822 - t42823 + t42824 - t42826 + t42828 + 0.1138200265427045984e0_f64 * t42829 + 0.1138200265427045984e0_f64 * t42832 + 0.1138200265427045984e0_f64 * t42835 + t42838 + t42841 - t42844 - t42845 + t42847 + t42850 - 0.12646669615856066489e-1_f64 * t42852 + t42857 + 0.28455006635676149599e-1_f64 * t1063 * t1064 * t42086;
    t42861
}
