//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1335/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1335<F: Float>(t537: F, t9947: F, t113: F, t2115: F, t1604: F, t3053: F, t910: F, t538: F, t6155: F, t2625: F, t3071: F, t28320: F, t7338: F, t19793: F, t20499: F, t24049: F, t24052: F, t25243: F, t27966: F, t27973: F, t27975: F, t27979: F, t27983: F, t5109: F, t6139: F, t7987: F, t8770: F, t9280: F) -> (F, F, F, F, F, F) {
    let t32664 = t537 * t9947;
    let t32665 = t32664 * t113;
    let t32666 = t2115 * t32665;
    let t32667 = t1604 * t32666;
    let t32669 = t3053 * t910;
    let t32671 = t6155 * t538 * t32669;
    let t32675 = t3071 * t2625;
    let t32679 = t7338 * t28320;
    let t32692 = 0.27439371595564631661e-2 * t32667 - 0.16463622957338778996e-1 * t32671 - 0.78013995660488417067e0 * t25243 * t8770 + 0.31205598264195366828e1 * t20499 * t5109 * t32675 - 0.15602799132097683414e1 * t6139 * t5109 * t32679 + 0.7801399566048841707e0 * t7987 * t9280 - 0.12459097221822660494e0 * t19793 + 0.34672886960217074253e0 * t27966 + 0.38415120233790484327e1 * t27973 - 0.11426392607441748233e0 * t27975 - 0.57131963037208741166e-1 * t27979 + 0.24393601348456957547e-3 * t27983 + 0.59329162131926993721e1 * t24049 - t24052;
    (t32664, t32666, t32669, t32675, t32679, t32692)
}
