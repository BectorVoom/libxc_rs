//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta588 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1941;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1942;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1943;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1944;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1945;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1946;
use chunk6::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1947;
use chunk7::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1948;
use chunk8::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1949;
use chunk9::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1950;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta588<F: Float>(t1513: F, t94975: F, t28036: F, t94978: F, t25823: F, t4287: F, t2340: F, t94982: F, t665: F, t25826: F, t2366: F, t13509: F, t6998: F, t1353: F, t22496: F, t13514: F, t93: F, t116: F, t28683: F, t2055: F, t2371: F, t1459: F, t1461: F, t1518: F, t18214: F, t1916: F, t1918: F, t2113: F, t2327: F, t26716: F, t26730: F, t26734: F, t26737: F, t28956: F, t28975: F, t28978: F, t28981: F, t28986: F, t4158: F, t4165: F, t572: F, t5795: F, t670: F, t7554: F, t7983: F, t8118: F, t8124: F, t8127: F, t7373: F, t94976: F, t94979: F, t94981: F, t95397: F, t114: F, t7968: F, t26179: F, t28133: F, t7706: F, t95293: F, t60224: F, t7342: F, t13272: F, t26178: F, t6960: F, t26205: F, t7709: F, t101129: F, t101132: F, t101139: F, t101337: F, t2048: F, t25159: F, t26175: F, t28116: F, t28119: F, t7352: F, t95310: F, t28640: F, t6963: F, t28141: F, t7349: F, t101350: F, t10309: F, t25120: F, t26172: F, t28147: F, t33269: F, t7343: F, t7964: F, t95230: F, t95241: F, t95243: F, t95246: F, t95248: F, t95253: F, t101172: F, t101176: F, t101182: F, t101187: F, t101190: F, t101193: F, t101357: F, t26187: F, t28105: F, t28109: F, t28112: F, t95255: F, t95259: F, t101226: F, t2047: F, t95283: F, t101156: F, t101323: F, t25102: F, t25110: F, t25114: F, t25162: F, t28602: F, t28635: F, t26169: F, t60221: F, t95268: F, t95270: F, t95284: F, t95286: F, t95288: F, t95290: F, t95294: F, t28093: F, t7702: F, t6954: F, t1923: F, t28089: F, t7348: F, t101360: F, t25150: F, t95297: F, t95314: F, t95320: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t101451, t101453, t101455, t101458, t101461, t101464, t101466) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1941::<F>(t1513, t94975, t28036, t94978, t25823, t4287, t2340, t94982, t665, t25826, t2366, t13509, t6998);
        let (t101479, t101522, t101724) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1942::<F>(t1353, t22496, t13514, t93, t116, t28683, t2055, t2371, t1459, t1461, t1518, t18214, t1916, t1918, t2113, t2327, t26716, t26730, t26734, t26737, t28956, t28975, t28978, t28981, t28986, t4158, t4165, t572, t5795, t670, t7554, t7983, t8118, t8124, t8127);
        let (t101725, t101760) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1943::<F>(t670, t7373, t101451, t101453, t101455, t101458, t101461, t101464, t101466, t94976, t94979, t94981, t95397);
        let (t101761, t101767, t101782, t101783, t101785, t101790) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1944::<F>(t114, t101760, t2327, t7968, t26179, t28133, t7706, t95293, t60224, t7342, t13272, t26178, t6960);
        let t101805 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1945::<F>(t26205, t7709, t101129, t101132, t101139, t101337, t101782, t101783, t101785, t101790, t2048, t25159, t26175, t28116, t28119, t7352, t7706, t95310);
        let t101824 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1946::<F>(t28640, t6963, t28141, t7349, t101350, t10309, t25120, t26172, t28147, t33269, t7343, t7709, t7964, t95230, t95241, t95243, t95246, t95248, t95253);
        let t101849 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1947::<F>(t101172, t101176, t101182, t101187, t101190, t101193, t101357, t2048, t26187, t28105, t28109, t28112, t7343, t7352, t7706, t95255, t95259);
        let t101875 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1948::<F>(t101226, t2047, t7706, t95283, t26179, t28105, t28109, t101156, t101323, t2048, t25102, t25110, t25114, t25162, t26187, t28133, t28141, t28602, t28635, t6963, t7343, t7352, t7964);
        let t101896 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1949::<F>(t28112, t7349, t28116, t28119, t26169, t7709, t60221, t7342, t6960, t95268, t95270, t95284, t95286, t95288, t95290, t95294);
        let t101919 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1950::<F>(t28093, t7349, t26169, t7702, t28640, t6954, t1923, t28089, t7348, t26205, t101360, t2048, t25150, t26172, t7352, t7964, t95297, t95314, t95320);
    (t101479, t101522, t101724, t101725, t101761, t101767, t101805, t101824, t101849, t101875, t101896, t101919)
}
