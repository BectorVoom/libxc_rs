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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta588(t1513: f64, t94975: f64, t28036: f64, t94978: f64, t25823: f64, t4287: f64, t2340: f64, t94982: f64, t665: f64, t25826: f64, t2366: f64, t13509: f64, t6998: f64, t1353: f64, t22496: f64, t13514: f64, t93: f64, t116: f64, t28683: f64, t2055: f64, t2371: f64, t1459: f64, t1461: f64, t1518: f64, t18214: f64, t1916: f64, t1918: f64, t2113: f64, t2327: f64, t26716: f64, t26730: f64, t26734: f64, t26737: f64, t28956: f64, t28975: f64, t28978: f64, t28981: f64, t28986: f64, t4158: f64, t4165: f64, t572: f64, t5795: f64, t670: f64, t7554: f64, t7983: f64, t8118: f64, t8124: f64, t8127: f64, t7373: f64, t94976: f64, t94979: f64, t94981: f64, t95397: f64, t114: f64, t7968: f64, t26179: f64, t28133: f64, t7706: f64, t95293: f64, t60224: f64, t7342: f64, t13272: f64, t26178: f64, t6960: f64, t26205: f64, t7709: f64, t101129: f64, t101132: f64, t101139: f64, t101337: f64, t2048: f64, t25159: f64, t26175: f64, t28116: f64, t28119: f64, t7352: f64, t95310: f64, t28640: f64, t6963: f64, t28141: f64, t7349: f64, t101350: f64, t10309: f64, t25120: f64, t26172: f64, t28147: f64, t33269: f64, t7343: f64, t7964: f64, t95230: f64, t95241: f64, t95243: f64, t95246: f64, t95248: f64, t95253: f64, t101172: f64, t101176: f64, t101182: f64, t101187: f64, t101190: f64, t101193: f64, t101357: f64, t26187: f64, t28105: f64, t28109: f64, t28112: f64, t95255: f64, t95259: f64, t101226: f64, t2047: f64, t95283: f64, t101156: f64, t101323: f64, t25102: f64, t25110: f64, t25114: f64, t25162: f64, t28602: f64, t28635: f64, t26169: f64, t60221: f64, t95268: f64, t95270: f64, t95284: f64, t95286: f64, t95288: f64, t95290: f64, t95294: f64, t28093: f64, t7702: f64, t6954: f64, t1923: f64, t28089: f64, t7348: f64, t101360: f64, t25150: f64, t95297: f64, t95314: f64, t95320: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t101451, t101453, t101455, t101458, t101461, t101464, t101466) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1941(t1513, t94975, t28036, t94978, t25823, t4287, t2340, t94982, t665, t25826, t2366, t13509, t6998);
        let (t101479, t101522, t101724) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1942(t1353, t22496, t13514, t93, t116, t28683, t2055, t2371, t1459, t1461, t1518, t18214, t1916, t1918, t2113, t2327, t26716, t26730, t26734, t26737, t28956, t28975, t28978, t28981, t28986, t4158, t4165, t572, t5795, t670, t7554, t7983, t8118, t8124, t8127);
        let (t101725, t101760) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1943(t670, t7373, t101451, t101453, t101455, t101458, t101461, t101464, t101466, t94976, t94979, t94981, t95397);
        let (t101761, t101767, t101782, t101783, t101785, t101790) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1944(t114, t101760, t2327, t7968, t26179, t28133, t7706, t95293, t60224, t7342, t13272, t26178, t6960);
        let t101805 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1945(t26205, t7709, t101129, t101132, t101139, t101337, t101782, t101783, t101785, t101790, t2048, t25159, t26175, t28116, t28119, t7352, t7706, t95310);
        let t101824 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1946(t28640, t6963, t28141, t7349, t101350, t10309, t25120, t26172, t28147, t33269, t7343, t7709, t7964, t95230, t95241, t95243, t95246, t95248, t95253);
        let t101849 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1947(t101172, t101176, t101182, t101187, t101190, t101193, t101357, t2048, t26187, t28105, t28109, t28112, t7343, t7352, t7706, t95255, t95259);
        let t101875 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1948(t101226, t2047, t7706, t95283, t26179, t28105, t28109, t101156, t101323, t2048, t25102, t25110, t25114, t25162, t26187, t28133, t28141, t28602, t28635, t6963, t7343, t7352, t7964);
        let t101896 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1949(t28112, t7349, t28116, t28119, t26169, t7709, t60221, t7342, t6960, t95268, t95270, t95284, t95286, t95288, t95290, t95294);
        let t101919 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1950(t28093, t7349, t26169, t7702, t28640, t6954, t1923, t28089, t7348, t26205, t101360, t2048, t25150, t26172, t7352, t7964, t95297, t95314, t95320);
    (t101479, t101522, t101724, t101725, t101761, t101767, t101805, t101824, t101849, t101875, t101896, t101919)
}
