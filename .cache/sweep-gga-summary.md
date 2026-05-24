# Batched compile sweep — summary

- effective jobs: `3`
- effective batch_size: `20`
- families: `gga`
- total packages swept: `131`
- pass-1 successes: `131`
- pass-2 successes: `0`
- failures: `0`
- total wall-clock: `5615.0 s`
- peak-of-peak RSS: `29671.0 MB`

## Per-family

| family | ok | pass-2 | fail |
|---|---|---|---|
| gga | 131 | 0 | 0 |

## Per-batch timing + RSS

| batch_index | package_count | wall_time_s | peak_rss_mb | all_ok |
|---|---|---|---|---|
| 0 | 20 | 2061.7 | 29671.0 | True |
| 1 | 20 | 1266.3 | 16803.5 | True |
| 2 | 20 | 82.3 | 1286.2 | True |
| 3 | 20 | 161.4 | 3521.7 | True |
| 4 | 20 | 859.6 | 8950.6 | True |
| 5 | 20 | 220.1 | 3997.1 | True |
| 6 | 11 | 963.6 | 18832.0 | True |

VERDICT: ALL_OK
