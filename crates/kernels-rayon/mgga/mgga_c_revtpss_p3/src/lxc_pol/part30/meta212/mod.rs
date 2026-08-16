//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta212 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1009;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1010;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1011;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta212(t4669: f64, t954: f64, t1621: f64, t2970: f64, t953: f64, t2848: f64, t2974: f64, t4571: f64, t4576: f64, t4581: f64, t4585: f64, t324: f64, t1626: f64, t964: f64, t1634: f64, t972: f64, t2906: f64, t2994: f64, t3001: f64, t4599: f64, t4607: f64, t4615: f64, t4617: f64, t4620: f64, t4623: f64, t4626: f64, t4629: f64, t973: f64, t1633: f64, t3014: f64, t1622: f64, t2938: f64, t2943: f64, t2968: f64, t2982: f64, t2987: f64, t3012: f64, t311: f64, t4589: f64, t4592: f64, t4594: f64, t4597: f64, t4634: f64, t4638: f64, t4644: f64, t4647: f64, t4652: f64, t946: f64, t955: f64, t965: f64, t974: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4670, t4673, t4674, t4682, t4683) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1009(t4669, t954, t1621, t2970, t953, t2848, t2974, t4571, t4576, t4581, t4585, t324);
        let (t4685, t4690, t4707) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1010(t1626, t964, t1634, t972, t2848, t2906, t2994, t3001, t4571, t4576, t4581, t4585, t4599, t4607, t4615, t4617, t4620, t4623, t4626, t4629);
        let (t4708, t4711, t4712, t4715) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1011(t4707, t973, t1633, t3014, t972, t1622, t1634, t2938, t2943, t2968, t2982, t2987, t3012, t311, t4589, t4592, t4594, t4597, t4634, t4638, t4644, t4647, t4652, t4670, t4674, t4683, t4685, t4690, t946, t955, t965, t974);
    (t4670, t4673, t4674, t4682, t4683, t4685, t4690, t4707, t4708, t4711, t4712, t4715)
}
