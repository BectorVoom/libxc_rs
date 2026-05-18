//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1439/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1439<F: Float>(t17340: F, t4477: F, t59022: F, t935: F, t15274: F, t1529: F, t1541: F, t16011: F, t16024: F, t27202: F, t27209: F, t27644: F, t3107: F, t3234: F, t3235: F, t3244: F, t4289: F, t450: F, t45795: F, t45809: F, t5389: F, t5404: F, t54789: F, t54959: F, t55390: F, t55392: F, t55396: F, t55425: F, t59030: F, t59711: F) -> (F, F, F) {
    let t60041 = t17340 * t4477;
    let t60060 = t59022 * t935;
    let t60065 = F::new(0.20606012420240018619e0) * t45795 - F::new(0.1133330683113201024e1) * t45809 + F::new(0.1559479530529405812e2) * t55390 + F::new(0.15802725909364645561e4) * t55392 + F::new(0.3118959061058811624e2) * t55425 + F::new(0.1559479530529405812e2) * t3234 * t3235 * t59711 - F::new(0.12117441361606500412e2) * t3244 * t4289 * t60041 + F::new(0.26631068404529536697e4) * t27644 * t54959 * t15274 - F::new(0.20734288552082234039e3) * t54789 * t1529 + F::new(0.66645927488835752265e2) * t16011 * t5404 - F::new(0.57943328334337033725e4) * t55396 * t1541 + F::new(0.11852044432023484171e4) * t16024 * t5389 - F::new(0.27821325036192187983e8) * t27202 * t450 * t59030 * t935 + F::new(0.81145531355560548285e7) * t27209 * t450 * t60060 * t3107;
    (t60041, t60060, t60065)
}
