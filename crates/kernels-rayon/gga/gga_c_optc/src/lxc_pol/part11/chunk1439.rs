//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1439/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1439(t17340: f64, t4477: f64, t59022: f64, t935: f64, t15274: f64, t1529: f64, t1541: f64, t16011: f64, t16024: f64, t27202: f64, t27209: f64, t27644: f64, t3107: f64, t3234: f64, t3235: f64, t3244: f64, t4289: f64, t450: f64, t45795: f64, t45809: f64, t5389: f64, t5404: f64, t54789: f64, t54959: f64, t55390: f64, t55392: f64, t55396: f64, t55425: f64, t59030: f64, t59711: f64) -> (f64, f64, f64) {
    let t60041 = t17340 * t4477;
    let t60060 = t59022 * t935;
    let t60065 = 0.20606012420240018619e0_f64 * t45795 - 0.1133330683113201024e1_f64 * t45809 + 0.1559479530529405812e2_f64 * t55390 + 0.15802725909364645561e4_f64 * t55392 + 0.3118959061058811624e2_f64 * t55425 + 0.1559479530529405812e2_f64 * t3234 * t3235 * t59711 - 0.12117441361606500412e2_f64 * t3244 * t4289 * t60041 + 0.26631068404529536697e4_f64 * t27644 * t54959 * t15274 - 0.20734288552082234039e3_f64 * t54789 * t1529 + 0.66645927488835752265e2_f64 * t16011 * t5404 - 0.57943328334337033725e4_f64 * t55396 * t1541 + 0.11852044432023484171e4_f64 * t16024 * t5389 - 0.27821325036192187983e8_f64 * t27202 * t450 * t59030 * t935 + 0.81145531355560548285e7_f64 * t27209 * t450 * t60060 * t3107;
    (t60041, t60060, t60065)
}
